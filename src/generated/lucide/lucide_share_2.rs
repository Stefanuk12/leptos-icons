use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" cx = "18" r = "3" ></ circle > < circle cx = "6" r = "3" cy = "12" ></ circle > < circle cy = "19" cx = "18" r = "3" ></ circle > < line y2 = "17.49" y1 = "13.51" x1 = "8.59" x2 = "15.42" ></ line > < line y2 = "10.49" y1 = "6.51" x1 = "15.41" x2 = "8.59" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;