use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z" ></ path > < path d = "M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z" ></ path > < path d = "M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12" ></ path > < path d = "M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z" ></ path > < / > } } pub const LUCIDE_CHERRY : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round")] } ;