use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 12h3a2 2 0 0 0 1.902-1.38l1.056-3.333A1 1 0 0 0 21 6H3a1 1 0 0 0-.958 1.287l1.056 3.334A2 2 0 0 0 5 12h3" ></ path > < path d = "M18 6V3a1 1 0 0 0-1-1h-3" ></ path > < rect width = "8" rx = "1" x = "8" y = "10" height = "12" ></ rect > < / > } } pub const LUCIDE_LECTERN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;