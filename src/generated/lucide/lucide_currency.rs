use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "8" ></ circle > < line x2 = "6" x1 = "3" y1 = "3" y2 = "6" ></ line > < line x2 = "18" x1 = "21" y1 = "3" y2 = "6" ></ line > < line y2 = "18" x2 = "6" y1 = "21" x1 = "3" ></ line > < line y1 = "21" x1 = "21" y2 = "18" x2 = "18" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;