# Use the official Nginx base image
FROM nginx:alpine

# Copy the static site content into the appropriate directory
# Assuming your static files are in a directory named "dist" or "public"
COPY ./.output/public /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/sites-enabled/default

# Expose port 80 to serve HTTP traffic
EXPOSE 80

# Start the Nginx server
CMD ["nginx", "-g", "daemon off;"]
