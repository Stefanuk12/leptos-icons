use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M145.5 0H224h78.5l44.7 50.8s39.3-10.9 57.8 7.6s33.8 34.9 33.8 34.9l-12 29.5 15.3 43.7s-44.9 170.2-50.1 191c-10.4 40.9-17.4 56.8-46.9 77.5s-82.9 56.8-91.6 62.2c-1.9 1.2-3.9 2.5-5.9 3.9c-7.5 5.1-15.8 10.8-23.5 10.8l0 0 0 0c-7.7 0-16.1-5.7-23.5-10.8c-2-1.4-4-2.8-5.9-3.9c-8.7-5.5-62.1-41.5-91.6-62.2s-36.5-36.6-46.9-77.5c-5.3-20.8-50.1-191-50.1-191l15.3-43.7L9.2 93.3s15.3-16.4 33.8-34.9s57.8-7.6 57.8-7.6L145.5 0zM224 407.6l0 0c3.7 0 8.9-4.7 13-8.4c.6-.5 1.2-1.1 1.7-1.5c4.2-3.7 47.8-37.5 51-39.8s5.4-6.5 1.9-8.7c-2.8-1.7-10-5.5-20.3-10.8c-3-1.6-6.3-3.2-9.7-5c-15.4-8-34.5-14.7-37.5-14.7l0 0 0 0c-3 0-22.1 6.8-37.5 14.7c-3.5 1.8-6.7 3.5-9.7 5c-10.3 5.3-17.6 9.1-20.3 10.8c-3.6 2.2-1.4 6.4 1.9 8.7s46.8 36.1 51 39.8c.5 .5 1.1 1 1.7 1.5c4.1 3.7 9.3 8.4 13 8.4l0 0zm0-165.7l0 0c4.7 0 17.6-3 26.4-5l0 0 2-.5c7.8-1.8 7.3-6.3 6.4-13c-.1-.8-.2-1.6-.3-2.4c-.6-6.1-5.8-33.1-9.1-50.3c-1.1-5.8-2-10.5-2.4-12.9c-1.5-8.1-.6-9.4 .7-11.3c.2-.3 .5-.7 .7-1.1c1.4-2.3 16-6.2 27.9-9.5l0 0c2.5-.7 4.8-1.3 6.9-1.9c10.6-3 32.4-.6 44.2 .6c1.8 .2 3.4 .4 4.7 .5c9.6 .9 10.4 2.3 7.2 3.8c-2.3 1.1-16.2 6.3-28.7 10.9l0 0 0 0c-4.7 1.8-9.2 3.5-12.8 4.8c-1.5 .5-3 1.1-4.5 1.7c-12.5 4.6-27.2 10-28.9 19.4c-1.5 8.3 5.2 19.9 11.3 30.3l0 0c1.6 2.8 3.2 5.5 4.6 8.1c6.3 11.9 6.5 13.3 6.1 18.1c-.4 3.9-14.5 12.7-22.4 17.6l0 0c-1.8 1.1-3.3 2.1-4.2 2.7c-.8 .5-2.1 1.4-3.8 2.4c-8.6 5.2-26.3 16-26.3 22.5c0 7.8 24.6 28.1 32.4 33.2s28.9 16.1 37.9 17.8s23-8.5 31.2-23.8c7.7-14.4 1.7-28.5-3.2-40l-.9-2.2c-4.5-10.6 1.9-17 6.2-21.3l0 0c.5-.5 1-1 1.4-1.4L377.7 194c1.3-1.3 2.5-2.6 3.7-3.8l0 0c5.8-5.7 10.8-10.5 10.8-22.8c0-14.9-57.5-84.5-57.5-84.5s-48.5 9.3-55.1 9.3c-5.2 0-15.3-3.5-25.8-7.1l0 0c-2.7-.9-5.4-1.9-8-2.7C232.8 78.1 224 78 224 78l0 0 0 0s-8.7 0-21.8 4.4c-2.7 .9-5.4 1.8-8 2.7l0 0c-10.5 3.6-20.6 7.1-25.8 7.1c-6.5 0-55.1-9.3-55.1-9.3s-57.5 69.6-57.5 84.5c0 12.3 4.9 17.1 10.8 22.8l0 0c1.2 1.2 2.5 2.4 3.7 3.8l43.1 45.8c.4 .5 .9 .9 1.4 1.4l0 0c4.3 4.3 10.6 10.7 6.2 21.3l-.9 2.2c-4.9 11.5-11 25.6-3.2 40c8.2 15.3 22.2 25.5 31.2 23.8s30.1-12.7 37.9-17.8s32.4-25.4 32.4-33.2c0-6.5-17.7-17.3-26.3-22.5c-1.7-1-3.1-1.9-3.8-2.4c-.9-.6-2.4-1.5-4.2-2.7c-7.9-4.9-22-13.7-22.4-17.6c-.4-4.8-.3-6.2 6.1-18.1c1.3-2.5 2.9-5.3 4.6-8.1c6-10.4 12.8-22 11.3-30.3c-1.7-9.4-16.4-14.8-28.9-19.4c-1.6-.6-3.1-1.1-4.5-1.7c-3.6-1.4-8.1-3.1-12.8-4.8l-.1 0c-12.5-4.7-26.4-9.9-28.7-10.9c-3.2-1.5-2.3-2.8 7.2-3.8c1.3-.1 2.9-.3 4.7-.5c11.8-1.3 33.6-3.6 44.2-.6c2.1 .6 4.4 1.2 6.9 1.9c11.9 3.2 26.5 7.2 27.9 9.5c.2 .4 .5 .7 .7 1.1c1.3 1.9 2.2 3.2 .7 11.3c-.4 2.4-1.3 7.1-2.4 12.9c-3.3 17.2-8.5 44.2-9.1 50.3c-.1 .8-.2 1.7-.3 2.4c-.8 6.7-1.4 11.2 6.4 13l2 .5 0 0c8.8 2 21.8 5 26.4 5l0 0z" ></ path > < / > } } pub const FONT_AWESOME_BRANDS_BRAVE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 448 512")] } ;