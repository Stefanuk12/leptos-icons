use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "7.5" r = ".5" fill = "currentColor" cy = "7.5" ></ circle > < circle cy = "5.5" fill = "currentColor" r = ".5" cx = "18.5" ></ circle > < circle cx = "11.5" r = ".5" cy = "11.5" fill = "currentColor" ></ circle > < circle r = ".5" fill = "currentColor" cx = "7.5" cy = "16.5" ></ circle > < circle fill = "currentColor" cx = "17.5" r = ".5" cy = "14.5" ></ circle > < path d = "M3 3v18h18" ></ path > < / > } } pub const LUCIDE_SCATTER_CHART : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24")] } ;