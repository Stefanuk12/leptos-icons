use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "8" cx = "12" ></ circle > < line y1 = "3" x2 = "6" x1 = "3" y2 = "6" ></ line > < line y1 = "3" x2 = "18" x1 = "21" y2 = "6" ></ line > < line x1 = "3" x2 = "6" y1 = "21" y2 = "18" ></ line > < line y1 = "21" x1 = "21" y2 = "18" x2 = "18" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;