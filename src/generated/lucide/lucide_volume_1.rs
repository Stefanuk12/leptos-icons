use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "11 5 6 9 2 9 2 15 6 15 11 19 11 5" ></ polygon > < path d = "M15.54 8.46a5 5 0 0 1 0 7.07" ></ path > < / > } } pub const LUCIDE_VOLUME_1 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor")] } ;