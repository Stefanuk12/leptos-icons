use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 6h4" ></ path > < path d = "M2 10h4" ></ path > < path d = "M2 14h4" ></ path > < path d = "M2 18h4" ></ path > < rect y = "2" height = "20" rx = "2" width = "16" x = "4" ></ rect > < path d = "M9.5 8h5" ></ path > < path d = "M9.5 12H16" ></ path > < path d = "M9.5 16H14" ></ path > < / > } } pub const LUCIDE_NOTEBOOK_TEXT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24")] } ;