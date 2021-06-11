import React from 'react';
import { EuiPanel, EuiFlexGroup, EuiFlexItem, EuiFormRow, EuiCheckbox, EuiListGroup, EuiFlexGrid, EuiListGroupItem, EuiText, EuiSpacer } from '@elastic/eui';

export default class Network extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        let config = this.props.config;
        return (
            <EuiFlexGroup gutterSize="none" responsive={false}>
                <EuiFlexItem>
                    <EuiFlexGrid columns={2} gutterSize="none" responsive={false}>
                        <EuiFlexItem><EuiText size="s">Name</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.name}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Status</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.status}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Ethernet MAC</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText className="font-monospaced" size="s">{config.mac}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Virtual NIC Device</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.portDeviceName}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Virtual NIC MTU</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.mtu}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Ethernet Broadcast</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.broadcastEnabled ? 'enabled' : 'disabled'}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Ethernet Bridging</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.bridge ? 'allowed' : 'prohibited'}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">DNS Domain</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{(((config.dns||{}).domain) ? config.dns.domain : '(not configured)')}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">DNS Servers</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{((((config.dns||{}).servers||[]).length > 0) ? config.dns.servers.map((server) => { return <span key={server}>{server}</span>; }) : '(none)')}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiSpacer size="s"/></EuiFlexItem>
                        <EuiFlexItem><EuiSpacer size="s"/></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Allow Managed IPs</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowManaged} onChange={(e) => {  }}/></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Allow Global Internet IPs</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowGlobal} onChange={(e) => {  }}/></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Allow Default Route Override</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowDefault} onChange={(e) => {  }}/></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Allow DNS Configuration</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowDNS} onChange={(e) => {  }}/></EuiFlexItem>
                    </EuiFlexGrid>
                </EuiFlexItem>
                <EuiFlexItem>
                    <EuiFormRow label="Managed IPs">
                        <div style={{overflow: 'hidden'}}>
                            <EuiPanel className="eui-yScroll" paddingSize="s" border={false} hasShadow={false} style={{height: 'calc(4em + 8px)'}}>
                                <EuiText size="xs">
                                    {(config.assignedAddresses||[]).map((address) => {
                                        return <div className="font-monospaced" key={address}>{address}</div>;
                                    })}
                                </EuiText>
                            </EuiPanel>
                        </div>
                    </EuiFormRow>
                    <EuiFormRow label="Managed Routes">
                        <div style={{overflow: 'hidden'}}>
                            <EuiPanel className="eui-yScroll" paddingSize="s" border={false} hasShadow={false} style={{height: 'calc(4em + 8px)'}}>
                                <EuiText size="xs" style={{whiteSpace: 'nowrap', overflowX: 'hidden'}}>
                                    {(config.routes||[]).map((route) => {
                                        let via = route.via||'(lan)';
                                        return <div className="font-monospaced" key={(route.target+via)}>{route.target} via {via}</div>;
                                    })}
                                </EuiText>
                            </EuiPanel>
                        </div>
                    </EuiFormRow>
                    <EuiFormRow label="Ethernet Multicast Subscriptions">
                        <div style={{overflow: 'hidden'}}>
                            <EuiPanel className="eui-yScroll" paddingSize="s" border={false} hasShadow={false} style={{height: 'calc(4em + 8px)'}}>
                                <EuiText size="xs" style={{whiteSpace: 'nowrap', overflowX: 'hidden'}}>
                                    {(config.multicastSubscriptions||[]).map((ms) => {
                                        let adi = 0;
                                        try {
                                            adi = parseInt(ms.adi);
                                        } catch (e) {}
                                        adi = (adi > 0) ? '/'+adi.toString(16) : '';
                                        return <div className="font-monospaced" key={(ms.mac+adi)}>{ms.mac}{adi}</div>;
                                    })}
                                </EuiText>
                            </EuiPanel>
                        </div>
                    </EuiFormRow>
                </EuiFlexItem>
            </EuiFlexGroup>
        );
    }
}
