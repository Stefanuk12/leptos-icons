use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8h20" ></ path > < rect height = "16" x = "2" width = "20" y = "4" rx = "2" ></ rect > < path d = "M6 16h12" ></ path > < / > } } pub const LUCIDE_DOCK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2")] } ;