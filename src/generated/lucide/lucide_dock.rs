use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8h20" ></ path > < rect height = "16" x = "2" rx = "2" y = "4" width = "20" ></ rect > < path d = "M6 16h12" ></ path > < / > } } pub const LUCIDE_DOCK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;