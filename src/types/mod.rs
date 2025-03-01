
pub use self::_common::{
  RObject,
  RFunction,
  detect_td_type,
  detect_td_type_and_extra,
  from_json,
  TdType,
};

#[macro_use] mod _common;

pub use self::account_ttl::*;
pub use self::address::*;
pub use self::animated_chat_photo::*;
pub use self::animated_emoji::*;
pub use self::animation::*;
pub use self::animations::*;
pub use self::audio::*;
pub use self::authentication_code_info::*;
pub use self::authentication_code_type::*;
pub use self::authorization_state::*;
pub use self::auto_download_settings::*;
pub use self::auto_download_settings_presets::*;
pub use self::background::*;
pub use self::background_fill::*;
pub use self::background_type::*;
pub use self::backgrounds::*;
pub use self::bank_card_action_open_url::*;
pub use self::bank_card_info::*;
pub use self::basic_group::*;
pub use self::basic_group_full_info::*;
pub use self::bot_command::*;
pub use self::bot_command_scope::*;
pub use self::bot_commands::*;
pub use self::call::*;
pub use self::call_discard_reason::*;
pub use self::call_id::*;
pub use self::call_problem::*;
pub use self::call_protocol::*;
pub use self::call_server::*;
pub use self::call_server_type::*;
pub use self::call_state::*;
pub use self::callback_query_answer::*;
pub use self::callback_query_payload::*;
pub use self::can_transfer_ownership_result::*;
pub use self::chat::*;
pub use self::chat_action::*;
pub use self::chat_action_bar::*;
pub use self::chat_administrator::*;
pub use self::chat_administrators::*;
pub use self::chat_event::*;
pub use self::chat_event_action::*;
pub use self::chat_event_log_filters::*;
pub use self::chat_events::*;
pub use self::chat_filter::*;
pub use self::chat_filter_info::*;
pub use self::chat_invite_link::*;
pub use self::chat_invite_link_count::*;
pub use self::chat_invite_link_counts::*;
pub use self::chat_invite_link_info::*;
pub use self::chat_invite_link_member::*;
pub use self::chat_invite_link_members::*;
pub use self::chat_invite_links::*;
pub use self::chat_join_request::*;
pub use self::chat_join_requests::*;
pub use self::chat_join_requests_info::*;
pub use self::chat_list::*;
pub use self::chat_lists::*;
pub use self::chat_location::*;
pub use self::chat_member::*;
pub use self::chat_member_status::*;
pub use self::chat_members::*;
pub use self::chat_members_filter::*;
pub use self::chat_nearby::*;
pub use self::chat_notification_settings::*;
pub use self::chat_permissions::*;
pub use self::chat_photo::*;
pub use self::chat_photo_info::*;
pub use self::chat_photos::*;
pub use self::chat_position::*;
pub use self::chat_report_reason::*;
pub use self::chat_source::*;
pub use self::chat_statistics::*;
pub use self::chat_statistics_administrator_actions_info::*;
pub use self::chat_statistics_inviter_info::*;
pub use self::chat_statistics_message_interaction_info::*;
pub use self::chat_statistics_message_sender_info::*;
pub use self::chat_theme::*;
pub use self::chat_type::*;
pub use self::chats::*;
pub use self::chats_nearby::*;
pub use self::check_chat_username_result::*;
pub use self::check_sticker_set_name_result::*;
pub use self::closed_vector_path::*;
pub use self::color_replacement::*;
pub use self::connected_website::*;
pub use self::connected_websites::*;
pub use self::connection_state::*;
pub use self::contact::*;
pub use self::count::*;
pub use self::countries::*;
pub use self::country_info::*;
pub use self::custom_request_result::*;
pub use self::database_statistics::*;
pub use self::date::*;
pub use self::date_range::*;
pub use self::dated_file::*;
pub use self::deep_link_info::*;
pub use self::device_token::*;
pub use self::dice_stickers::*;
pub use self::document::*;
pub use self::draft_message::*;
pub use self::email_address_authentication_code_info::*;
pub use self::emojis::*;
pub use self::encrypted_credentials::*;
pub use self::encrypted_passport_element::*;
pub use self::error::*;
pub use self::file::*;
pub use self::file_part::*;
pub use self::file_type::*;
pub use self::formatted_text::*;
pub use self::found_messages::*;
pub use self::functions::*;
pub use self::game::*;
pub use self::game_high_score::*;
pub use self::game_high_scores::*;
pub use self::group_call::*;
pub use self::group_call_id::*;
pub use self::group_call_participant::*;
pub use self::group_call_participant_video_info::*;
pub use self::group_call_recent_speaker::*;
pub use self::group_call_video_quality::*;
pub use self::group_call_video_source_group::*;
pub use self::hashtags::*;
pub use self::http_url::*;
pub use self::identity_document::*;
pub use self::imported_contacts::*;
pub use self::inline_keyboard_button::*;
pub use self::inline_keyboard_button_type::*;
pub use self::inline_query_result::*;
pub use self::inline_query_results::*;
pub use self::input_background::*;
pub use self::input_chat_photo::*;
pub use self::input_credentials::*;
pub use self::input_file::*;
pub use self::input_identity_document::*;
pub use self::input_inline_query_result::*;
pub use self::input_message_content::*;
pub use self::input_passport_element::*;
pub use self::input_passport_element_error::*;
pub use self::input_passport_element_error_source::*;
pub use self::input_personal_document::*;
pub use self::input_sticker::*;
pub use self::input_thumbnail::*;
pub use self::internal_link_type::*;
pub use self::invoice::*;
pub use self::json_object_member::*;
pub use self::json_value::*;
pub use self::keyboard_button::*;
pub use self::keyboard_button_type::*;
pub use self::labeled_price_part::*;
pub use self::language_pack_info::*;
pub use self::language_pack_string::*;
pub use self::language_pack_string_value::*;
pub use self::language_pack_strings::*;
pub use self::local_file::*;
pub use self::localization_target_info::*;
pub use self::location::*;
pub use self::log_stream::*;
pub use self::log_tags::*;
pub use self::log_verbosity_level::*;
pub use self::login_url_info::*;
pub use self::mask_point::*;
pub use self::mask_position::*;
pub use self::message::*;
pub use self::message_calendar::*;
pub use self::message_calendar_day::*;
pub use self::message_content::*;
pub use self::message_copy_options::*;
pub use self::message_file_type::*;
pub use self::message_forward_info::*;
pub use self::message_forward_origin::*;
pub use self::message_interaction_info::*;
pub use self::message_link::*;
pub use self::message_link_info::*;
pub use self::message_position::*;
pub use self::message_positions::*;
pub use self::message_reply_info::*;
pub use self::message_scheduling_state::*;
pub use self::message_send_options::*;
pub use self::message_sender::*;
pub use self::message_senders::*;
pub use self::message_sending_state::*;
pub use self::message_statistics::*;
pub use self::message_thread_info::*;
pub use self::messages::*;
pub use self::minithumbnail::*;
pub use self::network_statistics::*;
pub use self::network_statistics_entry::*;
pub use self::network_type::*;
pub use self::notification::*;
pub use self::notification_group::*;
pub use self::notification_group_type::*;
pub use self::notification_settings_scope::*;
pub use self::notification_type::*;
pub use self::ok::*;
pub use self::option_value::*;
pub use self::order_info::*;
pub use self::page_block::*;
pub use self::page_block_caption::*;
pub use self::page_block_horizontal_alignment::*;
pub use self::page_block_list_item::*;
pub use self::page_block_related_article::*;
pub use self::page_block_table_cell::*;
pub use self::page_block_vertical_alignment::*;
pub use self::passport_authorization_form::*;
pub use self::passport_element::*;
pub use self::passport_element_error::*;
pub use self::passport_element_error_source::*;
pub use self::passport_element_type::*;
pub use self::passport_elements::*;
pub use self::passport_elements_with_errors::*;
pub use self::passport_required_element::*;
pub use self::passport_suitable_element::*;
pub use self::password_state::*;
pub use self::payment_form::*;
pub use self::payment_form_theme::*;
pub use self::payment_receipt::*;
pub use self::payment_result::*;
pub use self::payments_provider_stripe::*;
pub use self::personal_details::*;
pub use self::personal_document::*;
pub use self::phone_number_authentication_settings::*;
pub use self::phone_number_info::*;
pub use self::photo::*;
pub use self::photo_size::*;
pub use self::point::*;
pub use self::poll::*;
pub use self::poll_option::*;
pub use self::poll_type::*;
pub use self::profile_photo::*;
pub use self::proxies::*;
pub use self::proxy::*;
pub use self::proxy_type::*;
pub use self::public_chat_type::*;
pub use self::push_message_content::*;
pub use self::push_receiver_id::*;
pub use self::recommended_chat_filter::*;
pub use self::recommended_chat_filters::*;
pub use self::recovery_email_address::*;
pub use self::remote_file::*;
pub use self::reply_markup::*;
pub use self::reset_password_result::*;
pub use self::rich_text::*;
pub use self::saved_credentials::*;
pub use self::scope_notification_settings::*;
pub use self::search_messages_filter::*;
pub use self::seconds::*;
pub use self::secret_chat::*;
pub use self::secret_chat_state::*;
pub use self::session::*;
pub use self::sessions::*;
pub use self::shipping_option::*;
pub use self::sponsored_message::*;
pub use self::sponsored_messages::*;
pub use self::statistical_graph::*;
pub use self::statistical_value::*;
pub use self::sticker::*;
pub use self::sticker_set::*;
pub use self::sticker_set_info::*;
pub use self::sticker_sets::*;
pub use self::stickers::*;
pub use self::storage_statistics::*;
pub use self::storage_statistics_by_chat::*;
pub use self::storage_statistics_by_file_type::*;
pub use self::storage_statistics_fast::*;
pub use self::suggested_action::*;
pub use self::supergroup::*;
pub use self::supergroup_full_info::*;
pub use self::supergroup_members_filter::*;
pub use self::t_me_url::*;
pub use self::t_me_url_type::*;
pub use self::t_me_urls::*;
pub use self::tdlib_parameters::*;
pub use self::temporary_password_state::*;
pub use self::terms_of_service::*;
pub use self::test_bytes::*;
pub use self::test_int::*;
pub use self::test_string::*;
pub use self::test_vector_int::*;
pub use self::test_vector_int_object::*;
pub use self::test_vector_string::*;
pub use self::test_vector_string_object::*;
pub use self::text::*;
pub use self::text_entities::*;
pub use self::text_entity::*;
pub use self::text_entity_type::*;
pub use self::text_parse_mode::*;
pub use self::theme_settings::*;
pub use self::thumbnail::*;
pub use self::thumbnail_format::*;
pub use self::top_chat_category::*;
pub use self::update::*;
pub use self::updates::*;
pub use self::user::*;
pub use self::user_full_info::*;
pub use self::user_privacy_setting::*;
pub use self::user_privacy_setting_rule::*;
pub use self::user_privacy_setting_rules::*;
pub use self::user_status::*;
pub use self::user_type::*;
pub use self::users::*;
pub use self::validated_order_info::*;
pub use self::vector_path_command::*;
pub use self::venue::*;
pub use self::video::*;
pub use self::video_chat::*;
pub use self::video_note::*;
pub use self::voice_note::*;
pub use self::web_page::*;
pub use self::web_page_instant_view::*;




mod account_ttl;
mod address;
mod animated_chat_photo;
mod animated_emoji;
mod animation;
mod animations;
mod audio;
mod authentication_code_info;
mod authentication_code_type;
mod authorization_state;
mod auto_download_settings;
mod auto_download_settings_presets;
mod background;
mod background_fill;
mod background_type;
mod backgrounds;
mod bank_card_action_open_url;
mod bank_card_info;
mod basic_group;
mod basic_group_full_info;
mod bot_command;
mod bot_command_scope;
mod bot_commands;
mod call;
mod call_discard_reason;
mod call_id;
mod call_problem;
mod call_protocol;
mod call_server;
mod call_server_type;
mod call_state;
mod callback_query_answer;
mod callback_query_payload;
mod can_transfer_ownership_result;
mod chat;
mod chat_action;
mod chat_action_bar;
mod chat_administrator;
mod chat_administrators;
mod chat_event;
mod chat_event_action;
mod chat_event_log_filters;
mod chat_events;
mod chat_filter;
mod chat_filter_info;
mod chat_invite_link;
mod chat_invite_link_count;
mod chat_invite_link_counts;
mod chat_invite_link_info;
mod chat_invite_link_member;
mod chat_invite_link_members;
mod chat_invite_links;
mod chat_join_request;
mod chat_join_requests;
mod chat_join_requests_info;
mod chat_list;
mod chat_lists;
mod chat_location;
mod chat_member;
mod chat_member_status;
mod chat_members;
mod chat_members_filter;
mod chat_nearby;
mod chat_notification_settings;
mod chat_permissions;
mod chat_photo;
mod chat_photo_info;
mod chat_photos;
mod chat_position;
mod chat_report_reason;
mod chat_source;
mod chat_statistics;
mod chat_statistics_administrator_actions_info;
mod chat_statistics_inviter_info;
mod chat_statistics_message_interaction_info;
mod chat_statistics_message_sender_info;
mod chat_theme;
mod chat_type;
mod chats;
mod chats_nearby;
mod check_chat_username_result;
mod check_sticker_set_name_result;
mod closed_vector_path;
mod color_replacement;
mod connected_website;
mod connected_websites;
mod connection_state;
mod contact;
mod count;
mod countries;
mod country_info;
mod custom_request_result;
mod database_statistics;
mod date;
mod date_range;
mod dated_file;
mod deep_link_info;
mod device_token;
mod dice_stickers;
mod document;
mod draft_message;
mod email_address_authentication_code_info;
mod emojis;
mod encrypted_credentials;
mod encrypted_passport_element;
mod error;
mod file;
mod file_part;
mod file_type;
mod formatted_text;
mod found_messages;
mod functions;
mod game;
mod game_high_score;
mod game_high_scores;
mod group_call;
mod group_call_id;
mod group_call_participant;
mod group_call_participant_video_info;
mod group_call_recent_speaker;
mod group_call_video_quality;
mod group_call_video_source_group;
mod hashtags;
mod http_url;
mod identity_document;
mod imported_contacts;
mod inline_keyboard_button;
mod inline_keyboard_button_type;
mod inline_query_result;
mod inline_query_results;
mod input_background;
mod input_chat_photo;
mod input_credentials;
mod input_file;
mod input_identity_document;
mod input_inline_query_result;
mod input_message_content;
mod input_passport_element;
mod input_passport_element_error;
mod input_passport_element_error_source;
mod input_personal_document;
mod input_sticker;
mod input_thumbnail;
mod internal_link_type;
mod invoice;
mod json_object_member;
mod json_value;
mod keyboard_button;
mod keyboard_button_type;
mod labeled_price_part;
mod language_pack_info;
mod language_pack_string;
mod language_pack_string_value;
mod language_pack_strings;
mod local_file;
mod localization_target_info;
mod location;
mod log_stream;
mod log_tags;
mod log_verbosity_level;
mod login_url_info;
mod mask_point;
mod mask_position;
mod message;
mod message_calendar;
mod message_calendar_day;
mod message_content;
mod message_copy_options;
mod message_file_type;
mod message_forward_info;
mod message_forward_origin;
mod message_interaction_info;
mod message_link;
mod message_link_info;
mod message_position;
mod message_positions;
mod message_reply_info;
mod message_scheduling_state;
mod message_send_options;
mod message_sender;
mod message_senders;
mod message_sending_state;
mod message_statistics;
mod message_thread_info;
mod messages;
mod minithumbnail;
mod network_statistics;
mod network_statistics_entry;
mod network_type;
mod notification;
mod notification_group;
mod notification_group_type;
mod notification_settings_scope;
mod notification_type;
mod ok;
mod option_value;
mod order_info;
mod page_block;
mod page_block_caption;
mod page_block_horizontal_alignment;
mod page_block_list_item;
mod page_block_related_article;
mod page_block_table_cell;
mod page_block_vertical_alignment;
mod passport_authorization_form;
mod passport_element;
mod passport_element_error;
mod passport_element_error_source;
mod passport_element_type;
mod passport_elements;
mod passport_elements_with_errors;
mod passport_required_element;
mod passport_suitable_element;
mod password_state;
mod payment_form;
mod payment_form_theme;
mod payment_receipt;
mod payment_result;
mod payments_provider_stripe;
mod personal_details;
mod personal_document;
mod phone_number_authentication_settings;
mod phone_number_info;
mod photo;
mod photo_size;
mod point;
mod poll;
mod poll_option;
mod poll_type;
mod profile_photo;
mod proxies;
mod proxy;
mod proxy_type;
mod public_chat_type;
mod push_message_content;
mod push_receiver_id;
mod recommended_chat_filter;
mod recommended_chat_filters;
mod recovery_email_address;
mod remote_file;
mod reply_markup;
mod reset_password_result;
mod rich_text;
mod saved_credentials;
mod scope_notification_settings;
mod search_messages_filter;
mod seconds;
mod secret_chat;
mod secret_chat_state;
mod session;
mod sessions;
mod shipping_option;
mod sponsored_message;
mod sponsored_messages;
mod statistical_graph;
mod statistical_value;
mod sticker;
mod sticker_set;
mod sticker_set_info;
mod sticker_sets;
mod stickers;
mod storage_statistics;
mod storage_statistics_by_chat;
mod storage_statistics_by_file_type;
mod storage_statistics_fast;
mod suggested_action;
mod supergroup;
mod supergroup_full_info;
mod supergroup_members_filter;
mod t_me_url;
mod t_me_url_type;
mod t_me_urls;
mod tdlib_parameters;
mod temporary_password_state;
mod terms_of_service;
mod test_bytes;
mod test_int;
mod test_string;
mod test_vector_int;
mod test_vector_int_object;
mod test_vector_string;
mod test_vector_string_object;
mod text;
mod text_entities;
mod text_entity;
mod text_entity_type;
mod text_parse_mode;
mod theme_settings;
mod thumbnail;
mod thumbnail_format;
mod top_chat_category;
mod update;
mod updates;
mod user;
mod user_full_info;
mod user_privacy_setting;
mod user_privacy_setting_rule;
mod user_privacy_setting_rules;
mod user_status;
mod user_type;
mod users;
mod validated_order_info;
mod vector_path_command;
mod venue;
mod video;
mod video_chat;
mod video_note;
mod voice_note;
mod web_page;
mod web_page_instant_view;


