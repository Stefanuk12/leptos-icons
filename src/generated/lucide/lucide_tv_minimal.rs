use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 21h10" ></ path > < rect height = "14" y = "3" rx = "2" x = "2" width = "20" ></ rect > < / > } } pub const LUCIDE_TV_MINIMAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;