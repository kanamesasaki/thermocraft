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
    'site.description': 'Spacecraft Thermal Engineering Blog',
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
    'site.description': '宇宙機の熱制御と熱解析',
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
