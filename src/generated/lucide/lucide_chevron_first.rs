use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m17 18-6-6 6-6" ></ path > < path d = "M7 6v12" ></ path > < / > } } pub const LUCIDE_CHEVRON_FIRST : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;