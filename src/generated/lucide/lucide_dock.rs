use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8h20" ></ path > < rect x = "2" height = "16" y = "4" width = "20" rx = "2" ></ rect > < path d = "M6 16h12" ></ path > < / > } } pub const LUCIDE_DOCK : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;