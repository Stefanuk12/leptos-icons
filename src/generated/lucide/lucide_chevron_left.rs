use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 18-6-6 6-6" ></ path > < / > } } pub const LUCIDE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;