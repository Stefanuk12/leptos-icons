use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 21h10" ></ path > < rect height = "14" width = "20" y = "3" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_TV_MINIMAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;