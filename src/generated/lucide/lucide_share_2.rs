use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "5" cx = "18" ></ circle > < circle cx = "6" cy = "12" r = "3" ></ circle > < circle cx = "18" cy = "19" r = "3" ></ circle > < line y1 = "13.51" x2 = "15.42" x1 = "8.59" y2 = "17.49" ></ line > < line y2 = "10.49" y1 = "6.51" x1 = "15.41" x2 = "8.59" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;