use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 16.326A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.973" ></ path > < path d = "m13 12-3 5h4l-3 5" ></ path > < / > } } pub const LUCIDE_CLOUD_LIGHTNING : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;