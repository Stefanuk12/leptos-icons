use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 6h4" ></ path > < path d = "M2 10h4" ></ path > < path d = "M2 14h4" ></ path > < path d = "M2 18h4" ></ path > < rect rx = "2" width = "16" x = "4" height = "20" y = "2" ></ rect > < path d = "M9.5 8h5" ></ path > < path d = "M9.5 12H16" ></ path > < path d = "M9.5 16H14" ></ path > < / > } } pub const LUCIDE_NOTEBOOK_TEXT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;