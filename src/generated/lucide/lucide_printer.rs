use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" ></ path > < path d = "M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6" ></ path > < rect height = "8" x = "6" y = "14" width = "12" rx = "1" ></ rect > < / > } } pub const LUCIDE_PRINTER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;