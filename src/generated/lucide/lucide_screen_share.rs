use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3" ></ path > < path d = "M8 21h8" ></ path > < path d = "M12 17v4" ></ path > < path d = "m17 8 5-5" ></ path > < path d = "M17 3h5v5" ></ path > < / > } } pub const LUCIDE_SCREEN_SHARE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;