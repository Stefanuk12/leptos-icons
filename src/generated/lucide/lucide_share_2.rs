use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "5" r = "3" ></ circle > < circle cx = "6" cy = "12" r = "3" ></ circle > < circle cx = "18" cy = "19" r = "3" ></ circle > < line x2 = "15.42" x1 = "8.59" y1 = "13.51" y2 = "17.49" ></ line > < line x2 = "8.59" x1 = "15.41" y2 = "10.49" y1 = "6.51" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;