use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle fill = "currentColor" cx = "7.5" cy = "7.5" r = ".5" ></ circle > < circle cy = "5.5" cx = "18.5" r = ".5" fill = "currentColor" ></ circle > < circle fill = "currentColor" cx = "11.5" r = ".5" cy = "11.5" ></ circle > < circle r = ".5" cy = "16.5" cx = "7.5" fill = "currentColor" ></ circle > < circle cy = "14.5" cx = "17.5" r = ".5" fill = "currentColor" ></ circle > < path d = "M3 3v18h18" ></ path > < / > } } pub const LucideScatterChart : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;