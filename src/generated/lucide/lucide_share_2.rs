use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "3" cx = "18" ></ circle > < circle cx = "6" cy = "12" r = "3" ></ circle > < circle cy = "19" cx = "18" r = "3" ></ circle > < line y1 = "13.51" y2 = "17.49" x1 = "8.59" x2 = "15.42" ></ line > < line y1 = "6.51" x1 = "15.41" x2 = "8.59" y2 = "10.49" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;