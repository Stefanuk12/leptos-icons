use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" width = "20" x = "2" rx = "2" height = "16" ></ rect > < path d = "M10 4v4" ></ path > < path d = "M2 8h20" ></ path > < path d = "M6 4v4" ></ path > < / > } } pub const LUCIDE_APP_WINDOW : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;