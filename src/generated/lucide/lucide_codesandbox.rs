use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" ></ path > < polyline points = "7.5 4.21 12 6.81 16.5 4.21" ></ polyline > < polyline points = "7.5 19.79 7.5 14.6 3 12" ></ polyline > < polyline points = "21 12 16.5 14.6 16.5 19.79" ></ polyline > < polyline points = "3.27 6.96 12 12.01 20.73 6.96" ></ polyline > < line y1 = "22.08" x1 = "12" x2 = "12" y2 = "12" ></ line > < / > } } pub const LucideCodesandbox : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;