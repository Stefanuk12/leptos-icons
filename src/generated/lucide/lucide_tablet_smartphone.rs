use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" x = "3" width = "10" y = "8" rx = "2" ></ rect > < path d = "M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4" ></ path > < path d = "M8 18h.01" ></ path > < / > } } pub const LUCIDE_TABLET_SMARTPHONE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;