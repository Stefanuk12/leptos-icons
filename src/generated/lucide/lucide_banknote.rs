use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" x = "2" y = "6" rx = "2" width = "20" ></ rect > < circle cx = "12" cy = "12" r = "2" ></ circle > < path d = "M6 12h.01M18 12h.01" ></ path > < / > } } pub const LUCIDE_BANKNOTE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;