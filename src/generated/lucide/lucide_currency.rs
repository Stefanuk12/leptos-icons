use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "8" cx = "12" ></ circle > < line x2 = "6" y2 = "6" x1 = "3" y1 = "3" ></ line > < line y1 = "3" x2 = "18" x1 = "21" y2 = "6" ></ line > < line x2 = "6" y1 = "21" y2 = "18" x1 = "3" ></ line > < line y1 = "21" x2 = "18" x1 = "21" y2 = "18" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;