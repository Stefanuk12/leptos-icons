use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "5" r = "3" ></ circle > < circle cx = "6" cy = "12" r = "3" ></ circle > < circle cx = "18" cy = "19" r = "3" ></ circle > < line x1 = "8.59" y2 = "17.49" y1 = "13.51" x2 = "15.42" ></ line > < line y1 = "6.51" x2 = "8.59" x1 = "15.41" y2 = "10.49" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24")] } ;