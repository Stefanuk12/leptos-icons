use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" x = "2" y = "4" rx = "2" width = "20" ></ rect > < path d = "M10 4v4" ></ path > < path d = "M2 8h20" ></ path > < path d = "M6 4v4" ></ path > < / > } } pub const LUCIDE_APP_WINDOW : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;