use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" r = "3" cy = "5" ></ circle > < circle cx = "6" r = "3" cy = "12" ></ circle > < circle r = "3" cy = "19" cx = "18" ></ circle > < line x2 = "15.42" y1 = "13.51" x1 = "8.59" y2 = "17.49" ></ line > < line y2 = "10.49" x2 = "8.59" y1 = "6.51" x1 = "15.41" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;