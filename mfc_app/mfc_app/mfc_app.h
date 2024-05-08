
// mfc_app.h : PROJECT_NAME アプリケーションのメイン ヘッダー ファイルです
//

#pragma once

#ifndef __AFXWIN_H__
	#error "PCH に対してこのファイルをインクルードする前に 'pch.h' をインクルードしてください"
#endif

#include "resource.h"		// メイン シンボル


// CmfcappApp:
// このクラスの実装については、mfc_app.cpp を参照してください
//

class CmfcappApp : public CWinApp
{
public:
	CmfcappApp();

// オーバーライド
public:
	virtual BOOL InitInstance();

// 実装

	DECLARE_MESSAGE_MAP()
};

extern CmfcappApp theApp;
