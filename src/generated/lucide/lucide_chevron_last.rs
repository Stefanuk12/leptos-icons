use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 18 6-6-6-6" ></ path > < path d = "M17 6v12" ></ path > < / > } } pub const LUCIDE_CHEVRON_LAST : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;