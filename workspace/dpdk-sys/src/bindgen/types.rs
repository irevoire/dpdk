// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


include!("types/MARKER.rs");
include!("types/__virtio16.rs");
include!("types/__virtio32.rs");
include!("types/__virtio64.rs");
include!("types/_bindgen_ty_1.rs");
include!("types/_bindgen_ty_10.rs");
include!("types/_bindgen_ty_11.rs");
include!("types/_bindgen_ty_12.rs");
include!("types/_bindgen_ty_13.rs");
include!("types/_bindgen_ty_14.rs");
include!("types/_bindgen_ty_15.rs");
include!("types/_bindgen_ty_16.rs");
include!("types/_bindgen_ty_2.rs");
include!("types/_bindgen_ty_3.rs");
include!("types/_bindgen_ty_4.rs");
include!("types/_bindgen_ty_5.rs");
include!("types/_bindgen_ty_8.rs");
include!("types/_bindgen_ty_9.rs");
include!("types/arg_handler_t.rs");
include!("types/buffer_tx_error_fn.rs");
include!("types/cryptodev_close_t.rs");
include!("types/cryptodev_configure_t.rs");
include!("types/cryptodev_info_get_t.rs");
include!("types/cryptodev_queue_pair_count_t.rs");
include!("types/cryptodev_queue_pair_release_t.rs");
include!("types/cryptodev_queue_pair_setup_t.rs");
include!("types/cryptodev_queue_pair_start_t.rs");
include!("types/cryptodev_queue_pair_stop_t.rs");
include!("types/cryptodev_start_t.rs");
include!("types/cryptodev_stats_get_t.rs");
include!("types/cryptodev_stats_reset_t.rs");
include!("types/cryptodev_stop_t.rs");
include!("types/cryptodev_sym_configure_session_t.rs");
include!("types/cryptodev_sym_create_session_pool_t.rs");
include!("types/cryptodev_sym_free_session_t.rs");
include!("types/cryptodev_sym_get_session_private_size_t.rs");
include!("types/cryptodev_sym_queue_pair_attach_session_t.rs");
include!("types/cryptodev_sym_queue_pair_detach_session_t.rs");
include!("types/dequeue_pkt_burst_t.rs");
include!("types/efd_hashfunc_t.rs");
include!("types/efd_lookuptbl_t.rs");
include!("types/efd_value_t.rs");
include!("types/enqueue_pkt_burst_t.rs");
include!("types/eth_allmulticast_disable_t.rs");
include!("types/eth_allmulticast_enable_t.rs");
include!("types/eth_dev_close_t.rs");
include!("types/eth_dev_configure_t.rs");
include!("types/eth_dev_infos_get_t.rs");
include!("types/eth_dev_led_off_t.rs");
include!("types/eth_dev_led_on_t.rs");
include!("types/eth_dev_pci_callback_t.rs");
include!("types/eth_dev_reset_t.rs");
include!("types/eth_dev_set_link_down_t.rs");
include!("types/eth_dev_set_link_up_t.rs");
include!("types/eth_dev_start_t.rs");
include!("types/eth_dev_stop_t.rs");
include!("types/eth_dev_supported_ptypes_get_t.rs");
include!("types/eth_filter_ctrl_t.rs");
include!("types/eth_fw_version_get_t.rs");
include!("types/eth_get_dcb_info.rs");
include!("types/eth_get_eeprom_length_t.rs");
include!("types/eth_get_eeprom_t.rs");
include!("types/eth_get_reg_t.rs");
include!("types/eth_is_removed_t.rs");
include!("types/eth_l2_tunnel_eth_type_conf_t.rs");
include!("types/eth_l2_tunnel_offload_set_t.rs");
include!("types/eth_link_update_t.rs");
include!("types/eth_mac_addr_add_t.rs");
include!("types/eth_mac_addr_remove_t.rs");
include!("types/eth_mac_addr_set_t.rs");
include!("types/eth_mirror_rule_reset_t.rs");
include!("types/eth_mirror_rule_set_t.rs");
include!("types/eth_mtr_ops_get_t.rs");
include!("types/eth_pool_ops_supported_t.rs");
include!("types/eth_promiscuous_disable_t.rs");
include!("types/eth_promiscuous_enable_t.rs");
include!("types/eth_queue_release_t.rs");
include!("types/eth_queue_start_t.rs");
include!("types/eth_queue_stats_mapping_set_t.rs");
include!("types/eth_queue_stop_t.rs");
include!("types/eth_rx_burst_t.rs");
include!("types/eth_rx_descriptor_done_t.rs");
include!("types/eth_rx_descriptor_status_t.rs");
include!("types/eth_rx_disable_intr_t.rs");
include!("types/eth_rx_enable_intr_t.rs");
include!("types/eth_rx_queue_count_t.rs");
include!("types/eth_rx_queue_setup_t.rs");
include!("types/eth_rxq_info_get_t.rs");
include!("types/eth_set_eeprom_t.rs");
include!("types/eth_set_mc_addr_list_t.rs");
include!("types/eth_set_queue_rate_limit_t.rs");
include!("types/eth_stats_get_t.rs");
include!("types/eth_stats_reset_t.rs");
include!("types/eth_timesync_adjust_time.rs");
include!("types/eth_timesync_disable_t.rs");
include!("types/eth_timesync_enable_t.rs");
include!("types/eth_timesync_read_rx_timestamp_t.rs");
include!("types/eth_timesync_read_time.rs");
include!("types/eth_timesync_read_tx_timestamp_t.rs");
include!("types/eth_timesync_write_time.rs");
include!("types/eth_tm_ops_get_t.rs");
include!("types/eth_tx_burst_t.rs");
include!("types/eth_tx_descriptor_status_t.rs");
include!("types/eth_tx_done_cleanup_t.rs");
include!("types/eth_tx_prep_t.rs");
include!("types/eth_tx_queue_setup_t.rs");
include!("types/eth_txq_info_get_t.rs");
include!("types/eth_uc_all_hash_table_set_t.rs");
include!("types/eth_uc_hash_table_set_t.rs");
include!("types/eth_udp_tunnel_port_add_t.rs");
include!("types/eth_udp_tunnel_port_del_t.rs");
include!("types/eth_xstats_get_by_id_t.rs");
include!("types/eth_xstats_get_names_by_id_t.rs");
include!("types/eth_xstats_get_names_t.rs");
include!("types/eth_xstats_get_t.rs");
include!("types/eth_xstats_reset_t.rs");
include!("types/event_dequeue_burst_t.rs");
include!("types/event_dequeue_t.rs");
include!("types/event_enqueue_burst_t.rs");
include!("types/event_enqueue_t.rs");
include!("types/eventdev_close_t.rs");
include!("types/eventdev_configure_t.rs");
include!("types/eventdev_dequeue_timeout_ticks_t.rs");
include!("types/eventdev_dump_t.rs");
include!("types/eventdev_eth_rx_adapter_caps_get_t.rs");
include!("types/eventdev_eth_rx_adapter_queue_add_t.rs");
include!("types/eventdev_eth_rx_adapter_queue_del_t.rs");
include!("types/eventdev_eth_rx_adapter_start_t.rs");
include!("types/eventdev_eth_rx_adapter_stats_get.rs");
include!("types/eventdev_eth_rx_adapter_stats_reset.rs");
include!("types/eventdev_eth_rx_adapter_stop_t.rs");
include!("types/eventdev_info_get_t.rs");
include!("types/eventdev_pmd_pci_callback_t.rs");
include!("types/eventdev_port_default_conf_get_t.rs");
include!("types/eventdev_port_link_t.rs");
include!("types/eventdev_port_release_t.rs");
include!("types/eventdev_port_setup_t.rs");
include!("types/eventdev_port_unlink_t.rs");
include!("types/eventdev_queue_default_conf_get_t.rs");
include!("types/eventdev_queue_release_t.rs");
include!("types/eventdev_queue_setup_t.rs");
include!("types/eventdev_selftest.rs");
include!("types/eventdev_start_t.rs");
include!("types/eventdev_stop_t.rs");
include!("types/eventdev_xstats_get_by_name.rs");
include!("types/eventdev_xstats_get_names_t.rs");
include!("types/eventdev_xstats_get_t.rs");
include!("types/eventdev_xstats_reset_t.rs");
include!("types/flow_ctrl_get_t.rs");
include!("types/flow_ctrl_set_t.rs");
include!("types/hash_sig_t.rs");
include!("types/lcore_function_t.rs");
include!("types/member_set_t.rs");
include!("types/mtu_set_t.rs");
include!("types/pci_probe_t.rs");
include!("types/pci_remove_t.rs");
include!("types/phys_addr_t.rs");
include!("types/priority_flow_ctrl_set_t.rs");
include!("types/rawdev_close_t.rs");
include!("types/rawdev_configure_t.rs");
include!("types/rawdev_dequeue_bufs_t.rs");
include!("types/rawdev_dump_t.rs");
include!("types/rawdev_enqueue_bufs_t.rs");
include!("types/rawdev_firmware_load_t.rs");
include!("types/rawdev_firmware_status_get_t.rs");
include!("types/rawdev_firmware_unload_t.rs");
include!("types/rawdev_firmware_version_get_t.rs");
include!("types/rawdev_get_attr_t.rs");
include!("types/rawdev_info_get_t.rs");
include!("types/rawdev_queue_conf_get_t.rs");
include!("types/rawdev_queue_release_t.rs");
include!("types/rawdev_queue_setup_t.rs");
include!("types/rawdev_reset_t.rs");
include!("types/rawdev_selftest_t.rs");
include!("types/rawdev_set_attr_t.rs");
include!("types/rawdev_start_t.rs");
include!("types/rawdev_stop_t.rs");
include!("types/rawdev_xstats_get_by_name_t.rs");
include!("types/rawdev_xstats_get_names_t.rs");
include!("types/rawdev_xstats_get_t.rs");
include!("types/rawdev_xstats_reset_t.rs");
include!("types/reta_query_t.rs");
include!("types/reta_update_t.rs");
include!("types/rss_hash_conf_get_t.rs");
include!("types/rss_hash_update_t.rs");
include!("types/rte_bbdev_cb_fn.rs");
include!("types/rte_bbdev_close_t.rs");
include!("types/rte_bbdev_dequeue_dec_ops_t.rs");
include!("types/rte_bbdev_dequeue_enc_ops_t.rs");
include!("types/rte_bbdev_enqueue_dec_ops_t.rs");
include!("types/rte_bbdev_enqueue_enc_ops_t.rs");
include!("types/rte_bbdev_info_get_t.rs");
include!("types/rte_bbdev_intr_enable_t.rs");
include!("types/rte_bbdev_op_te_flag_bitmasks.rs");
include!("types/rte_bbdev_queue_intr_disable_t.rs");
include!("types/rte_bbdev_queue_intr_enable_t.rs");
include!("types/rte_bbdev_queue_release_t.rs");
include!("types/rte_bbdev_queue_setup_t.rs");
include!("types/rte_bbdev_queue_start_t.rs");
include!("types/rte_bbdev_queue_stop_t.rs");
include!("types/rte_bbdev_setup_queues_t.rs");
include!("types/rte_bbdev_start_t.rs");
include!("types/rte_bbdev_stats_get_t.rs");
include!("types/rte_bbdev_stats_reset_t.rs");
include!("types/rte_bbdev_stop_t.rs");
include!("types/rte_be16_t.rs");
include!("types/rte_be32_t.rs");
include!("types/rte_be64_t.rs");
include!("types/rte_bus_cmp_t.rs");
include!("types/rte_bus_find_device_t.rs");
include!("types/rte_bus_get_iommu_class_t.rs");
include!("types/rte_bus_parse_t.rs");
include!("types/rte_bus_plug_t.rs");
include!("types/rte_bus_probe_t.rs");
include!("types/rte_bus_scan_t.rs");
include!("types/rte_bus_unplug_t.rs");
include!("types/rte_cpuset_t.rs");
include!("types/rte_crypto_auth_operation.rs");
include!("types/rte_crypto_cipher_operation.rs");
include!("types/rte_cryptodev_cb_fn.rs");
include!("types/rte_cryptodev_scheduler_burst_dequeue_t.rs");
include!("types/rte_cryptodev_scheduler_burst_enqueue_t.rs");
include!("types/rte_cryptodev_scheduler_config_option_get.rs");
include!("types/rte_cryptodev_scheduler_config_option_set.rs");
include!("types/rte_cryptodev_scheduler_config_queue_pair.rs");
include!("types/rte_cryptodev_scheduler_create_private_ctx.rs");
include!("types/rte_cryptodev_scheduler_slave_attach_t.rs");
include!("types/rte_cryptodev_scheduler_slave_detach_t.rs");
include!("types/rte_cryptodev_scheduler_start_t.rs");
include!("types/rte_cryptodev_scheduler_stop_t.rs");
include!("types/rte_dev_cmp_t.rs");
include!("types/rte_eal_alarm_callback.rs");
include!("types/rte_eth_bond_8023ad_ext_slowrx_fn.rs");
include!("types/rte_eth_dev_cb_fn.rs");
include!("types/rte_event_eth_rx_adapter_conf_cb.rs");
include!("types/rte_fbk_hash_fn.rs");
include!("types/rte_hash_cmp_eq_t.rs");
include!("types/rte_hash_function.rs");
include!("types/rte_intr_callback_fn.rs");
include!("types/rte_intr_event_cb_t.rs");
include!("types/rte_iova_t.rs");
include!("types/rte_job_update_period_cb_t.rs");
include!("types/rte_keepalive_failure_callback_t.rs");
include!("types/rte_keepalive_relay_callback_t.rs");
include!("types/rte_latency_stats_flow_type_fn.rs");
include!("types/rte_le16_t.rs");
include!("types/rte_le32_t.rs");
include!("types/rte_le64_t.rs");
include!("types/rte_mempool_alloc_t.rs");
include!("types/rte_mempool_ctor_t.rs");
include!("types/rte_mempool_dequeue_t.rs");
include!("types/rte_mempool_enqueue_t.rs");
include!("types/rte_mempool_free_t.rs");
include!("types/rte_mempool_get_capabilities_t.rs");
include!("types/rte_mempool_get_count.rs");
include!("types/rte_mempool_mem_cb_t.rs");
include!("types/rte_mempool_memchunk_free_cb_t.rs");
include!("types/rte_mempool_obj_cb_t.rs");
include!("types/rte_mempool_obj_ctor_t.rs");
include!("types/rte_mempool_ops_register_memory_area_t.rs");
include!("types/rte_meter_color.rs");
include!("types/rte_mp_t.rs");
include!("types/rte_mtr_capabilities_get_t.rs");
include!("types/rte_mtr_create_t.rs");
include!("types/rte_mtr_destroy_t.rs");
include!("types/rte_mtr_meter_disable_t.rs");
include!("types/rte_mtr_meter_dscp_table_update_t.rs");
include!("types/rte_mtr_meter_enable_t.rs");
include!("types/rte_mtr_meter_profile_add_t.rs");
include!("types/rte_mtr_meter_profile_delete_t.rs");
include!("types/rte_mtr_meter_profile_update_t.rs");
include!("types/rte_mtr_policer_actions_update_t.rs");
include!("types/rte_mtr_stats_read_t.rs");
include!("types/rte_mtr_stats_update_t.rs");
include!("types/rte_pipeline_port_in_action_handler.rs");
include!("types/rte_pipeline_port_out_action_handler.rs");
include!("types/rte_pipeline_table_action_handler_hit.rs");
include!("types/rte_pipeline_table_action_handler_miss.rs");
include!("types/rte_port_in_op_create.rs");
include!("types/rte_port_in_op_free.rs");
include!("types/rte_port_in_op_rx.rs");
include!("types/rte_port_in_op_stats_read.rs");
include!("types/rte_port_out_op_create.rs");
include!("types/rte_port_out_op_flush.rs");
include!("types/rte_port_out_op_free.rs");
include!("types/rte_port_out_op_stats_read.rs");
include!("types/rte_port_out_op_tx.rs");
include!("types/rte_port_out_op_tx_bulk.rs");
include!("types/rte_power_freq_change_t.rs");
include!("types/rte_power_freqs_t.rs");
include!("types/rte_power_get_freq_t.rs");
include!("types/rte_power_set_freq_t.rs");
include!("types/rte_rawdev_obj_t.rs");
include!("types/rte_rx_callback_fn.rs");
include!("types/rte_service_func.rs");
include!("types/rte_table_hash_op_hash.rs");
include!("types/rte_table_op_create.rs");
include!("types/rte_table_op_entry_add.rs");
include!("types/rte_table_op_entry_add_bulk.rs");
include!("types/rte_table_op_entry_delete.rs");
include!("types/rte_table_op_entry_delete_bulk.rs");
include!("types/rte_table_op_free.rs");
include!("types/rte_table_op_lookup.rs");
include!("types/rte_table_op_stats_read.rs");
include!("types/rte_timer_cb_t.rs");
include!("types/rte_tm_capabilities_get_t.rs");
include!("types/rte_tm_hierarchy_commit_t.rs");
include!("types/rte_tm_level_capabilities_get_t.rs");
include!("types/rte_tm_mark_ip_dscp_t.rs");
include!("types/rte_tm_mark_ip_ecn_t.rs");
include!("types/rte_tm_mark_vlan_dei_t.rs");
include!("types/rte_tm_node_add_t.rs");
include!("types/rte_tm_node_capabilities_get_t.rs");
include!("types/rte_tm_node_cman_update_t.rs");
include!("types/rte_tm_node_delete_t.rs");
include!("types/rte_tm_node_parent_update_t.rs");
include!("types/rte_tm_node_resume_t.rs");
include!("types/rte_tm_node_shaper_update_t.rs");
include!("types/rte_tm_node_shared_shaper_update_t.rs");
include!("types/rte_tm_node_shared_wred_context_update_t.rs");
include!("types/rte_tm_node_stats_read_t.rs");
include!("types/rte_tm_node_stats_update_t.rs");
include!("types/rte_tm_node_suspend_t.rs");
include!("types/rte_tm_node_type_get_t.rs");
include!("types/rte_tm_node_wfq_weight_mode_update_t.rs");
include!("types/rte_tm_node_wred_context_update_t.rs");
include!("types/rte_tm_shaper_profile_add_t.rs");
include!("types/rte_tm_shaper_profile_delete_t.rs");
include!("types/rte_tm_shared_shaper_add_update_t.rs");
include!("types/rte_tm_shared_shaper_delete_t.rs");
include!("types/rte_tm_shared_wred_context_add_update_t.rs");
include!("types/rte_tm_shared_wred_context_delete_t.rs");
include!("types/rte_tm_wred_profile_add_t.rs");
include!("types/rte_tm_wred_profile_delete_t.rs");
include!("types/rte_tx_callback_fn.rs");
include!("types/rte_usage_hook_t.rs");
include!("types/rte_v128s16_t.rs");
include!("types/rte_v128s32_t.rs");
include!("types/rte_v128s64_t.rs");
include!("types/rte_v128s8_t.rs");
include!("types/rte_v128u16_t.rs");
include!("types/rte_v128u32_t.rs");
include!("types/rte_v128u64_t.rs");
include!("types/rte_v128u8_t.rs");
include!("types/rte_v256s16_t.rs");
include!("types/rte_v256s32_t.rs");
include!("types/rte_v256s64_t.rs");
include!("types/rte_v256s8_t.rs");
include!("types/rte_v256u16_t.rs");
include!("types/rte_v256u32_t.rs");
include!("types/rte_v256u64_t.rs");
include!("types/rte_v256u8_t.rs");
include!("types/rte_v64s16_t.rs");
include!("types/rte_v64s32_t.rs");
include!("types/rte_v64s8_t.rs");
include!("types/rte_v64u16_t.rs");
include!("types/rte_v64u32_t.rs");
include!("types/rte_v64u8_t.rs");
include!("types/rte_vdev_probe_t.rs");
include!("types/rte_vdev_remove_t.rs");
include!("types/rte_vdev_scan_callback.rs");
include!("types/rte_xmm_t.rs");
include!("types/security_capabilities_get_t.rs");
include!("types/security_get_userdata_t.rs");
include!("types/security_session_create_t.rs");
include!("types/security_session_destroy_t.rs");
include!("types/security_session_get_size.rs");
include!("types/security_session_stats_get_t.rs");
include!("types/security_session_update_t.rs");
include!("types/security_set_pkt_metadata_t.rs");
include!("types/vlan_filter_set_t.rs");
include!("types/vlan_offload_set_t.rs");
include!("types/vlan_pvid_set_t.rs");
include!("types/vlan_strip_queue_set_t.rs");
include!("types/vlan_tpid_set_t.rs");
include!("types/xmm_t.rs");
