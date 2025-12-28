export function formatDate(isoDate: string): string {
  const dateObj = new Date(isoDate);
  return dateObj.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
}
