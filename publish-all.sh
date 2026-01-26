#!/bin/bash

function publish_crate() {
    pushd $1
    echo "##### $1 #####"
    cargo publish
    echo "##############"
    echo
    echo
    popd
}

publish_crate localesupport/cntp_localesupport
publish_crate i18n/cntp_i18n_core
publish_crate i18n/cntp_i18n_build_core
publish_crate i18n/cntp_i18n_parse
publish_crate i18n/cntp_i18n_gen
publish_crate i18n/cntp_i18n_macros
publish_crate i18n/cntp_i18n
publish_crate i18n/cargo_cntp_i18n

publish_crate cntp_config
publish_crate icon_tool/cntp_icon_tool_core
publish_crate icon_tool/cntp_icon_tool_macros
publish_crate deploy_tool/cntp_bundle_lib
publish_crate deploy_tool/cargo_cntp_deploy
publish_crate deploy_tool/cargo_cntp_bundle