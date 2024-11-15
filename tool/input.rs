import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { COL } from "./columns.config";

export default function SuperTransRate() {
  const actionRef = useRef<ActionType>();

  async function request(params: unknow): Promise<any> {
    return {
      data: [],
      success: true,
      total: 0,
    };
  }

  return (
    <ProTable
      rowKey="id"
      actionRef={actionRef}
      request={request}
      size="small"
      columns={COL as any}
      search={{
        optionRender(searchConfig, formProps, dom) {
          return dom.filter((item: any) => item?.key === "submit");
        },
      }}
    />
  );
}
