use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "3" cx = "18" ></ circle > < circle r = "3" cy = "12" cx = "6" ></ circle > < circle cx = "18" r = "3" cy = "19" ></ circle > < line y1 = "13.51" x2 = "15.42" x1 = "8.59" y2 = "17.49" ></ line > < line y2 = "10.49" x2 = "8.59" x1 = "15.41" y1 = "6.51" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;