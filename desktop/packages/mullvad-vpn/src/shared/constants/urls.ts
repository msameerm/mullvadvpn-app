// SocialCommand VPN support links. No links to other websites should be added.
export const urls = {
  purchase: 'https://wa.me/447598789403',
  faq: 'https://wa.me/447598789403',
  privacyGuide: 'https://wa.me/447598789403',
  download: 'https://wa.me/447598789403',
  lanShare: 'https://wa.me/447598789403',
} as const;

type BaseUrl = (typeof urls)[keyof typeof urls];
type ExtendedBaseUrl = `${BaseUrl}${string}`;
export type Url = BaseUrl | ExtendedBaseUrl;
