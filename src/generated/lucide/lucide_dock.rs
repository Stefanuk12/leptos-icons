use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8h20" ></ path > < rect rx = "2" y = "4" width = "20" height = "16" x = "2" ></ rect > < path d = "M6 16h12" ></ path > < / > } } pub const LUCIDE_DOCK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;