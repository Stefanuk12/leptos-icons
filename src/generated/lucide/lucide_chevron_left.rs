use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 18-6-6 6-6" ></ path > < / > } } pub const LUCIDE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;