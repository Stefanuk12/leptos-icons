use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "5" cx = "18" ></ circle > < circle cy = "12" cx = "6" r = "3" ></ circle > < circle r = "3" cx = "18" cy = "19" ></ circle > < line y1 = "13.51" x1 = "8.59" x2 = "15.42" y2 = "17.49" ></ line > < line y1 = "6.51" y2 = "10.49" x2 = "8.59" x1 = "15.41" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;