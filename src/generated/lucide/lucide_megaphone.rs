use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 11 18-5v12L3 14v-3z" ></ path > < path d = "M11.6 16.8a3 3 0 1 1-5.8-1.6" ></ path > < / > } } pub const LUCIDE_MEGAPHONE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;