import React, { useRef } from 'react';
import PullList from '@/components/PullList';
import ins, { Pagination, Data } from '@/components/PullList/index.d';
import { View } from '@tarojs/components';
import Empty from '@/components/Empty';

/** 测试数据 */
function testDataMethod({ pageNum, pageSize }: Pagination) {
  return {
    list: [],
    total: 0,
  };
  return Promise.resolve({
    list: Array(10)
      .fill(1)
      .map((item, index) => {
        const id = `test_${index + pageNum * pageSize}`;
        return {
          id: id,
        };
      }),
    total: 0,
  });
}

// 查询hook 文件拆分
export default function useQueryPullList(
  params: React.MutableRefObject<unknown>,
  renderRow: (d: any, i: number) => React.ReactNode
) {
  //
  const listApi = useRef<ins>();

  async function getDataMethod({
    pageNum,
    pageSize,
  }: Pagination): Promise<Data> {
    try {
      const res = await testDataMethod({ pageNum, pageSize });

      return {
        list: res.list,
        total: res.total,
      };
    } catch (error) {
      return {
        list: [],
        total: 0,
      };
    }
  }

  return {
    listApi: listApi,
    pullList: (
      <PullList
        apiRef={listApi}
        offsetBottom={0}
        defaultPageSize={10}
        renderRow={renderRow}
        rowKey="id"
        getDataMethod={getDataMethod}
        listStyle={{
          padding: `8px 0`,
        }}
        footerStyle={{
          height: '50px',
        }}
        footer={{
          loading: <View>努力加载中...</View>,
          loadmore: <View></View>,
        }}
        emptyPage={<Empty />}
      />
    ),
  };
}
