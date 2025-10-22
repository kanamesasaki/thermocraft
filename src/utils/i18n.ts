export const languages = {
  en: 'English',
  ja: '日本語',
};

export const defaultLang = 'en';

export const ui = {
  en: {
    'nav.home': 'Home',
    'nav.about': 'About',
    'nav.articles': 'Articles',
    'site.title': 'ThermoCraft',
    'site.subtitle': 'Spacecraft Thermal Engineering Blog',
    'share.title': 'Share',
    'share.copy': 'Copy link',
    'share.copied': 'Copied!',
    'share.share': 'Share',
    'share.x': 'Share on X',
    'share.linkedin': 'Share on LinkedIn',
    'site.description':
      'ThermoCraft is a technical blog featuring in-depth articles on thermal engineering, aerospace engineering, numerical analysis, and programming.',
    'posts.title': 'Posts',
    'posts.filter': 'Filter by tags',
    'posts.recent': 'Recent Articles',
    'posts.related': 'Related Articles',
    'about.title': 'About ThermoCraft',
    'filter.all': 'All',
  },
  ja: {
    'nav.home': 'Home',
    'nav.about': 'About',
    'nav.articles': 'Articles',
    'site.title': 'ThermoCraft',
    'site.subtitle': '宇宙機の熱設計と熱解析',
    'share.title': 'シェア',
    'share.copy': 'リンクをコピー',
    'share.copied': 'コピーしました',
    'share.share': '共有',
    'share.x': 'Xで共有',
    'share.linkedin': 'LinkedInで共有',
    'site.description':
      'ThermoCraftは、熱工学、宇宙工学、数値解析、プログラミングに関する記事を特集した技術ブログです。',
    'posts.title': 'Posts',
    'posts.filter': 'タグで絞り込み',
    'posts.recent': '最新記事',
    'posts.related': '関連記事',
    'about.title': 'About ThermoCraft',
    'filter.all': 'All',
  },
} as const;

export function getLangFromUrl(url: URL) {
  const [, lang] = url.pathname.split('/');
  if (lang in languages) return lang as keyof typeof languages;
  return defaultLang;
}

export function useTranslations(lang: keyof typeof languages) {
  return function t(key: keyof (typeof ui)[typeof defaultLang]) {
    return ui[lang][key] || ui[defaultLang][key];
  };
}

export function getLocalizedPath(path: string, lang: keyof typeof languages) {
  if (lang === defaultLang) return path;
  return `/${lang}${path === '/' ? '' : path}`;
}

export function getCollectionName(lang: keyof typeof languages) {
  return lang === 'ja' ? 'articles-ja' : 'articles';
}
