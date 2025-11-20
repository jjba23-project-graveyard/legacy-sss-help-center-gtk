# sss-help

# Copyright © Josep Bigorra <jjbigorra@gmail.com>

# sss-help is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# sss-help is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with sss-help.  If not, see <https://www.gnu.org/licenses/>.

COPYRIGHT_HOLDERS:="Josep Bigorra"
I18N_LOCATION := "resources/locale"
LOG_MSG_PREFIX:="\n\033[1m[INFO]["
LOG_MSG_SUFFIX_PRE_MSG:="]"
LOG_END:="\033[0m\n"

fmt:
	guix shell -m manifest.scm -- rustfmt --verbose \
		./src/main.rs ./src/keybinds.rs ./src/widgets.rs
run:
	RUST_LOG=debug guix shell -f guix.scm -- sss-help

define get-date-string
	$(shell date --rfc-3339=seconds)
endef

# sss-help-center-log-complete: Logs when a target has finished executing.
define sss-help-center-log-complete
	@-echo -e $(LOG_MSG_PREFIX)$(strip $(call get-date-string))$(LOG_MSG_SUFFIX_PRE_MSG) \
		"Completed work on Make target: $(1)" \
		$(LOG_END)
endef

# sss-help-center-log-starting: Logs when a target starts execution.
define sss-help-center-log-starting
	@-echo -e $(LOG_MSG_PREFIX)$(strip $(call get-date-string))$(LOG_MSG_SUFFIX_PRE_MSG) \
		"Begin working on Make target: $(1)" \
		$(LOG_END)
endef

# sss-help-center-log-info: Logs informational messages in custom format with highlight.
define sss-help-center-log-info
	@-echo -e $(LOG_MSG_PREFIX)$(strip $(call get-date-string))$(LOG_MSG_SUFFIX_PRE_MSG) \
		"$(1)" \
		$(LOG_END)
endef

# i18n helpers

# Define a function for compiling the translation file (.po) into a machine-readable (.mo) format
# Arguments: $(1) represents the language code (e.g., "en_US", "nl_NL", etc.)
# Use msgfmt to compile the .po file into a .mo file for the specified language
# The output .mo file will be placed in the LC_MESSAGES directory under the language's folder
define sss-help-center-msgfmt
	msgfmt -v -o $(I18N_LOCATION)/$(1)/LC_MESSAGES/sss-help-center.mo \
		$(I18N_LOCATION)/$(1)/LC_MESSAGES/sss-help-center.po
endef

# Define a function to initialize a new translation file from a template (.pot) for a specific language
# Arguments: $(1) represents the language code (e.g., "en_US", "nl_NL", etc.)
# Use msginit to generate a .po file from the .pot template for the specified language
# The -l option sets the language, and --no-translator means no translator is pre-filled in the .po file
# The generated .po file is placed in the LC_MESSAGES directory for the given language
define sss-help-center-msginit
	mkdir -p $(I18N_LOCATION)/$(1)/LC_MESSAGES
	msginit -l $(1).UTF8 -o $(I18N_LOCATION)/$(1)/LC_MESSAGES/sss-help-center.po \
		-i $(I18N_LOCATION)/sss-help-center.pot --no-translator
endef

# Define a function to merge translation updates from the template (.pot) into an existing .po file
# Arguments: $(1) represents the language code (e.g., "en_US", "nl_NL", etc.)
# Use msgmerge to update the specified language's .po file with new translations from the template (.pot)
# The -U flag updates the .po file in place
# This ensures the .po file stays up-to-date with the latest changes in the template
define sss-help-center-merge
	msgmerge -U $(I18N_LOCATION)/$(1)/LC_MESSAGES/sss-help-center.po \
		$(I18N_LOCATION)/sss-help-center.pot 
endef

# Define a function to extract translatable strings from Scheme source files using xgettext
# Arguments: $(1) should be either "--join-existing" for updating template
#   or "" to start from scratch.
# Run xgettext to extract strings marked for translation from Scheme source files
define sss-help-center-xgettext
	xgettext --default-domain=sss-help-center \
		--output=$(I18N_LOCATION)/sss-help-center.pot \
		--language=C \
		--keyword=gettext \
		--sort-by-file $(1) \
		--copyright-holder=$(COPYRIGHT_HOLDERS) \
		--package-name="sss-help-center" \
		src/*.rs \
		--from-code UTF-8
endef

# Compile translation files to .mo files
compile-i18n:
	$(call sss-help-center-log-info,"Compiling translation files")
	$(call sss-help-center-msgfmt,"en_US")
	$(call sss-help-center-msgfmt,"nl_NL")
	$(call sss-help-center-log-complete,"compile-i18n")

# Create new i18n template from source code
new-i18n:
	$(call sss-help-center-log-info,"Creating new i18n template from source code")
	$(call sss-help-center-xgettext)

# Update i18n template from source code
join-i18n:
	$(call sss-help-center-log-info,"Updating i18n template from source code")
	$(call sss-help-center-xgettext,"--join-existing")

# en_US - English language targets
init-i18n-en:
	$(call sss-help-center-msginit,"en_US")
merge-i18n-en:
	$(call sss-help-center-merge,"en_US")

# nl_NL - Dutch language targets
init-i18n-nl:
	$(call sss-help-center-msginit,"nl_NL")
merge-i18n-nl:
	$(call sss-help-center-merge,"nl_NL")

guix-build:
	guix build -f guix.scm -d

guix-install:
	sudo cp "$$(guix build -f guix.scm -q)/bin/sss-help" ~/.local/bin/
