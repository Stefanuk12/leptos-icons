use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M304 160c0-70.7-57.3-128-128-128l-32 0C73.3 32 16 89.3 16 160c0 34.6 13.7 66 36 89C20.5 272.3 0 309.8 0 352c0 70.7 57.3 128 128 128l64 0c70.7 0 128-57.3 128-128c0-42.2-20.5-79.7-52-103c22.3-23 36-54.4 36-89zM176.1 288l15.9 0c35.3 0 64 28.7 64 64s-28.7 64-64 64l-64 0c-35.3 0-64-28.7-64-64s28.7-64 64-64l15.9 0c0 0 .1 0 .1 0l32 0c0 0 .1 0 .1 0zm0-64c0 0 0 0 0 0l-32 0c0 0 0 0 0 0c-35.3 0-64-28.7-64-64c0-35.3 28.7-64 64-64l32 0c35.3 0 64 28.7 64 64c0 35.3-28.6 64-64 64z" ></ path > < / > } } pub const FONT_AWESOME_SOLID_8 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 320 512")] } ;