use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "5" x1 = "19" y2 = "19" x2 = "5" ></ line > < circle cx = "6.5" r = "2.5" cy = "6.5" ></ circle > < circle cy = "17.5" r = "2.5" cx = "17.5" ></ circle > < / > } } pub const LUCIDE_PERCENT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;