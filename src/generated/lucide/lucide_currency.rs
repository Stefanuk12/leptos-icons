use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cy = "12" cx = "12" ></ circle > < line y1 = "3" y2 = "6" x2 = "6" x1 = "3" ></ line > < line y2 = "6" x2 = "18" y1 = "3" x1 = "21" ></ line > < line y2 = "18" x1 = "3" y1 = "21" x2 = "6" ></ line > < line x2 = "18" y1 = "21" x1 = "21" y2 = "18" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;