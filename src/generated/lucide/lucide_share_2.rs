use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "5" r = "3" ></ circle > < circle cy = "12" cx = "6" r = "3" ></ circle > < circle cy = "19" cx = "18" r = "3" ></ circle > < line x2 = "15.42" y2 = "17.49" x1 = "8.59" y1 = "13.51" ></ line > < line y2 = "10.49" x2 = "8.59" y1 = "6.51" x1 = "15.41" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;