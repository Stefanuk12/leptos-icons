use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "5" cx = "18" ></ circle > < circle r = "3" cx = "6" cy = "12" ></ circle > < circle cx = "18" cy = "19" r = "3" ></ circle > < line y2 = "17.49" x1 = "8.59" x2 = "15.42" y1 = "13.51" ></ line > < line x1 = "15.41" y2 = "10.49" x2 = "8.59" y1 = "6.51" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;