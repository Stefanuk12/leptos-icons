use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8h20" ></ path > < rect width = "20" rx = "2" x = "2" y = "4" height = "16" ></ rect > < path d = "M6 16h12" ></ path > < / > } } pub const LUCIDE_DOCK : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;